import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { AdminService } from '../admin.service';
import { Router } from '@angular/router';
import { RessourceInfo } from '../ressource-info';

@Component({
  selector: 'app-resource-detail',
  templateUrl: './resource-detail.component.html',
  styleUrls: ['./resource-detail.component.scss']
})
export class ResourceDetailComponent {

  @Input() ressource!: RessourceInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  message : string =''

  constructor(protected adminService : AdminService, private modalService: NgbModal, protected router: Router) { }


  openWindowCustomClass(content: any) {

    this.modalService.open(content);
  }


  supprimer(){

    this.adminService.delete_ressource(this.ressource.bastion_id,this.ressource.id).subscribe({

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

    this.router.navigate([`/admin/bastions/${this.ressource.bastion_id}/${this.ressource.id}`]);
  }


}
