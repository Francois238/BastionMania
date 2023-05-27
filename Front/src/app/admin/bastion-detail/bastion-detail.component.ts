import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { AdminService } from '../admin.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-bastion-detail',
  templateUrl: './bastion-detail.component.html',
  styleUrls: ['./bastion-detail.component.scss']
})
export class BastionDetailComponent {

  @Input() bastion!: BastionInfo;
  @Output() newItemEvent = new EventEmitter<string>();
  message : string =''

  constructor(protected adminService : AdminService, private modalService: NgbModal, protected router: Router) { }


  openWindowCustomClass(content: any) {

    this.modalService.open(content);
  }


  supprimer(){

    this.adminService.delete_bastion(this.bastion.bastion_id).subscribe({

      next: (data : any) => {
        
        this.message="L'administrateur a bien été supprimé"
        this.newItemEvent.emit("delete");

        
      },
      error: (e) => {
        
        this.message="Impossible de supprimer l'administrateur"
      },
    })

  }

  getDetail(){

    this.router.navigate([`/admin/bastions/${this.bastion.bastion_id}`]);
  }

}