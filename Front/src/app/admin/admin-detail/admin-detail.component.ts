import { Component, EventEmitter, Input, Output } from '@angular/core';
import { AdminService } from '../admin.service';
import { AdminInfo } from '../admin-info';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'app-admin-detail',
  templateUrl: './admin-detail.component.html',
  styleUrls: ['./admin-detail.component.scss']
})
export class AdminDetailComponent {

  @Input() admin!: AdminInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  message : string =''

  constructor(protected adminService : AdminService, private modalService: NgbModal) { }


  openWindowCustomClass(content: any) {

    this.modalService.open(content);
  }


  supprimer(){

    this.adminService.delete_admin(this.admin.id).subscribe({

      next: (data : any) => {
        
        this.message="L'administrateur a bien été supprimé"
        this.newItemEvent.emit("delete");

        
      },
      error: (e) => {
        
        this.message="Impossible de supprimer l'administrateur"
      },
    })

  }

}
